/*
using System;
using System.Collections.Generic;
using System.Linq;

namespace SemIOLibrary.Collections
{
    public class ListWithMultiplicity<T> : List<T>
    {
        private List<T> _list;
        private uint _lowerBoundary;
        private uint _upperBoundary;

        public uint UpperBoundary => get

        ListWithMultiplicity(Multiplicity multiplicity)
        {
            MultiplicityOfList = multiplicity;
        }

        ListWithMultiplicity()
        {
            MultiplicityOfList = new Multiplicity(1);
        }

        public new T this[int index]
        {
            get =>_list[index];
            set => _list[index] = value;
        }

        public new void Add(T item)
        {
            IfMultiplicityIsExactSizeThrow(); 
            if (_list.Count == (MultiplicityOfList.UpperBoundary ?? uint.MaxValue))
                throw new Exception("Your upper boundary is already reached. Set it higher before adding this item.");
            _list.Add(item);
        }

        public new void Insert(int index, T item)
        {
            IfMultiplicityIsExactSizeThrow(); 
            if (_list.Count == (MultiplicityOfList.UpperBoundary ?? uint.MaxValue))
                throw new Exception("Your upper boundary is already reached. Set it higher before adding this item.");
            _list.Insert(index, item);
        }

        public new void InsertRange(int index, IEnumerable<T> collection)
        {
            IfMultiplicityIsExactSizeThrow();
            if (_list.Count + collection.Count() - 1 == (MultiplicityOfList.UpperBoundary ?? uint.MaxValue))
                throw new Exception("Your upper boundary is already reached. Set it higher before adding this item.");
            _list.InsertRange(index, collection);
        }

        public new void AddRange(IEnumerable<T> collection)
        {
            IfMultiplicityIsExactSizeThrow();
            if (_list.Count + collection.Count() - 1 == (MultiplicityOfList.UpperBoundary ?? uint.MaxValue))
                throw new Exception("Your upper boundary is already reached. Set it higher before adding this item.");
            _list.InsertRange(_list.Count, collection);
        }

        public new void Remove(T item)
        {
            IfMultiplicityIsExactSizeThrow();
            if (_list.Count == (MultiplicityOfList.LowerBoundary ?? 0))
                throw new Exception("Your lower boundary is already reached. Set it lower before adding this item.");
            _list.Remove(item);
        }

        public void IfMultiplicityIsExactSizeThrow()
        {
            if (MultiplicityOfList.IsExactSize)
                throw new Exception("This list has an exact size. You are not allowed to add or remove items but only modify them.");
        }
    }
}
*/
