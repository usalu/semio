using System;
using System.Collections.Generic;
using GraphQL;
using GraphQL.Client.Http;
using GraphQL.Client.Serializer.Newtonsoft;

namespace Semio
{
    public class Script
    {
        public int Id { get; set; }
        public string Name { get; set; }
        public string Explanation { get; set; }
        public ScriptKind Kind { get; set; }
        public string Url { get; set; }
        public int KitId { get; set; }
        public Kit Kit { get; set; }
        public List<Property> SynthesizedProperties { get; set; }
        public List<Type> PrototypedTypes { get; set; }
        public List<Formation> ChoreographedFormations { get; set; }
        public List<Formation> TransformedFormations { get; set; }
    }

    public enum ScriptKind
    {
        SYNTHESIS,
        PROTOTYPE,
        MODIFICATION,
        CHOREOGRAPHY,
        TRANSFORMATION
    }

    public class Property
    {
        public int Id { get; set; }
        public string Name { get; set; }
        public string Explanation { get; set; }
        public string Datatype { get; set; }
        public string Value { get; set; }
        public int? SynthesisScriptId { get; set; }
        public int? TypeId { get; set; }
        public int? PortId { get; set; }
        public Script SynthesisScript { get; set; }
        public Type Type { get; set; }
        public Port Port { get; set; }
    }

    public class Type
    {
        public int Id { get; set; }
        public string Name { get; set; }
        public string Explanation { get; set; }
        public int KitId { get; set; }
        public int? PrototypeScriptId { get; set; }
        public List<Port> Ports { get; set; }
        public Kit Kit { get; set; }
        public Script PrototypeScript { get; set; }
        public List<Property> Properties { get; set; }
    }

    public class Port
    {
        public int Id { get; set; }
        public string Name { get; set; }
        public string Explanation { get; set; }
        public float? OriginX { get; set; }
        public float? OriginY { get; set; }
        public float? OriginZ { get; set; }
        public float? XAxisX { get; set; }
        public float? XAxisY { get; set; }
        public float? XAxisZ { get; set; }
        public float? YAxisX { get; set; }
        public float? YAxisY { get; set; }
        public float? YAxisZ { get; set; }
        public float? ZAxisX { get; set; }
        public float? ZAxisY { get; set; }
        public float? ZAxisZ { get; set; }
        public int TypeId { get; set; }
        public Type Type { get; set; }
        public List<Property> Properties { get; set; }
        public List<Attraction> Attractings { get; set; }
        public List<Attraction> Attracteds { get; set; }
    }

    public class Piece
    {
        public int Id { get; set; }
        public int FormationId { get; set; }
        public List<Attraction> Attractings { get; set; }
        public List<Attraction> Attracteds { get; set; }
        public Formation Formation { get; set; }
    }

    public class Attraction
    {
        public int AttractingPieceId { get; set; }
        public int AttractingPieceTypePortId { get; set; }
        public int AttractedPieceId { get; set; }
        public int AttractedPieceTypePortId { get; set; }
        public int FormationId { get; set; }
        public Piece AttractingPiece { get; set; }
        public Port AttractingPieceTypePort { get; set; }
        public Piece AttractedPiece { get; set; }
        public Port AttractedPieceTypePort { get; set; }
        public Formation Formation { get; set; }
    }

    public class Formation
    {
        public int Id { get; set; }
        public string Name { get; set; }
        public string Explanation { get; set; }
        public int? ChoreographyScriptId { get; set; }
        public int? TransformationScriptId { get; set; }
        public int KitId { get; set; }
        public List<Piece> Pieces { get; set; }
        public List<Attraction> Attractions { get; set; }
        public Script ChoreographyScript { get; set; }
        public Script TransformationScript { get; set; }
        public Kit Kit { get; set; }
    }

    public class Kit
    {
        public int Id { get; set; }
        public static string Uri { get; set; }
        public string Name { get; set; }
        public string Explanation { get; set; }
        public List<Script> Scripts { get; set; }
        public List<Type> Types { get; set; }
        public List<Formation> Formations { get; set; }
    }

    public class GetKitsResponse
{
    public List<Kit> Kits { get; set; }
}

    public class Api : ICloneable
    {
        public Api()
        {
            Endpoint = "http://127.0.0.1:5000/graphql";
            Token = "";
            Client = new GraphQLHttpClient(Endpoint, new NewtonsoftJsonSerializer());
        }

        public Api(string endpoint, string token)
        {
            Endpoint = endpoint;
            Token = token;
            Client = new GraphQLHttpClient(Endpoint, new NewtonsoftJsonSerializer());
            Client.HttpClient.DefaultRequestHeaders.Add("Authorization", $"Bearer {Token}");
        }

        public GraphQLHttpClient Client { get; set; }
        public string Endpoint { get; set; }
        public string Token { get; set; }

        public object Clone()
        {
            return new Api(Endpoint, Token);
        }

        public override string ToString()
        {
            return $"Api(Endpoint: {Endpoint}, Token: {Token})";
        }

        public List<Kit> GetKits()
        {
            var query = new GraphQLRequest
            {
                Query = @"
                   query {
                       kits {
                           id
                           uri
                           name
                           explanation
                           scripts {
                               id
                               name
                               explanation
                               kind
                               url    
                           }
                           types {
                               id
                               name
                               explanation
                               ports {
                                   id
                                   name
                                   explanation
                                   originX
                                   originY
                                   originZ
                                   xAxisX
                                   xAxisY
                                   xAxisZ
                                   yAxisX
                                   yAxisY
                                   yAxisZ
                                   zAxisX
                                   zAxisY
                                   zAxisZ
                               }
                           }
                           formations {
                               id
                               name
                               explanation
                               pieces {
                                   id
                               }
                               attractions {
                                   attractingPieceId
                                   attractingPieceTypePortId
                                   attractedPieceId
                                   attractedPieceTypePortId
                               }
                           }
                       }
                   }
               "
            };
            var response = Client.SendQueryAsync<GetKitsResponse>(query).Result;
            return response.Data.Kits;
        }

        public Kit GetKit(int id)
        {
            var query = new GraphQLRequest
            {
                Query = @"
                   query($id: Int!) {
                       kit(id: $id) {
                           id
                           uri
                           name
                           explanation
                           scripts {
                               id
                               name
                               explanation
                               kind
                               url    
                           }
                           types {
                               id
                               name
                               explanation
                               ports {
                                   id
                                   name
                                   explanation
                                   originX
                                   originY
                                   originZ
                                   xAxisX
                                   xAxisY
                                   xAxisZ
                                   yAxisX
                                   yAxisY
                                   yAxisZ
                                   zAxisX
                                   zAxisY
                                   zAxisZ
                               }
                           }
                           formations {
                               id
                               name
                               explanation
                               pieces {
                                   id
                               }
                               attractions {
                                   attractingPieceId
                                   attractingPieceTypePortId
                                   attractedPieceId
                                   attractedPieceTypePortId
                               }
                           }
                       }
                   }
               ",
                Variables = new {id}
            };
            var response = Client.SendQueryAsync<GetKitsResponse>(query).Result;
            return response.Data.Kits[0];
        }

        public Kit GetLocalKit()
        {
            
        }
    }
}